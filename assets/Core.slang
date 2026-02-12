[vk::binding(0, 0)]
__DynamicResource<__DynamicResourceKind.Sampler> _samplers[];
[vk::binding(1, 0)]
__DynamicResource<__DynamicResourceKind.General> _textures[];

export struct Hnd<T : IOpaqueDescriptor>
{
    uint Id;

    T get()
    {
        switch (T.kind)
        {
            case DescriptorKind.Sampler:
                return _samplers[Id].asOpaqueDescriptor<T>();
            case DescriptorKind.Texture:
                return _textures[Id].asOpaqueDescriptor<T>();
            default:
                return _textures[Id].asOpaqueDescriptor<T>();
        }
    }
}
